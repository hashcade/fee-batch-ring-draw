"""Exercise native well menu callbacks without running Unity or granting items."""

from unicorn import arm64_const as reg
from collector_fixture import CollectorFixture


class WellFixture(CollectorFixture):
  def __init__(self, cpu, game, write, profile, common):
    super().__init__(cpu, game, write, profile, common)
    self.heap = game + 0x8300000
    self.functions[game + 0x025C9240] = "MESS_GET"

  def handle(self, pc, import_name):
    if not self.active: return False
    name = self.functions.get(pc, import_name)
    args = [self.cpu.reg_read(getattr(reg, f"UC_ARM64_REG_X{i}")) for i in range(4)]
    result = None
    if name == "CREATE_MENU": self.original_calls += 1
    elif name in ("INITIALIZE_METADATA", "WRITE_BARRIER", "ITEM_CTOR"): pass
    elif name == "ALLOCATE_OBJECT": result = self.alloc()
    elif name == "LIST_ADD":
      self.append(args[0], args[1])
      self.qword(args[1] + 0x10, self.menu)
    elif name == "SET_MENU_ROWS": self.rows = args[1]
    elif name == "USE_FLAG": result = self.flag
    elif name == "EXCHANGE_LEVEL": result = self.level
    elif name == "GET_SEED": result = self.seed
    elif name == "SET_SEED":
      self.seed = args[0] & 0xFFFFFFFF
      self.seeds.append(self.seed)
    elif name == "SYSTEM_RANDOM": result = self.system
    elif name == "RANDOM_VALUE":
      assert args[0] == self.system
      self.live_rng_calls += 1
      result = self.new_seed
    elif name == "RANDOM_CTOR":
      assert args[0] != self.system and args[1] == self.seed
      self.private_rng = args[0]
    elif name == "CALCULATE":
      assert args[:3] == [self.parent, self.level, self.private_rng]
      assert self.private_rng != self.system
      self.calculations += 1
      result = 0 if self.fail_calc else self.results
    elif name == "MESS_GET": result = self.string(self.read_string(args[0]).removeprefix("MID_"))
    elif name == "MESSAGE":
      assert args[0] == self.menu
      self.messages.append(self.read_string(args[1]))
      result = self.alloc()
      self.qword(self.menu + 0x38, result)
    elif name == "GET_LANGUAGE": result = self.language
    elif name == "STRING_NEW_SIZE":
      result = self.alloc(0x20 + args[0] * 2)
      self.integer(result + 0x10, args[0])
    else: return False
    if result is not None: self.cpu.reg_write(reg.UC_ARM64_REG_X0, result)
    self.cpu.reg_write(reg.UC_ARM64_REG_PC, self.cpu.reg_read(reg.UC_ARM64_REG_X30))
    return True

  def call(self, callback, *args):
    for index in range(8): self.cpu.reg_write(getattr(reg, f"UC_ARM64_REG_X{index}"), args[index] if index < len(args) else 0)
    self.cpu.reg_write(reg.UC_ARM64_REG_SP, self.stack)
    self.cpu.reg_write(reg.UC_ARM64_REG_X30, self.stop)
    self.cpu.emu_start(callback, self.stop + 4, count=200_000)
    assert self.cpu.reg_read(reg.UC_ARM64_REG_PC) == self.stop
    return self.cpu.reg_read(reg.UC_ARM64_REG_X0)

  def exercise(self, replacement, stop, stack):
    self.active, self.stop, self.stack = True, stop, stack
    self.reset()
    self.random_class = self.metadata(0x06287BF8)
    self.system = self.alloc()
    self.flag, self.level, self.seed, self.new_seed, self.language = 1, 5, 12345, 54321, 10
    self.fail_calc = False
    self.seeds, self.messages = [], []
    self.live_rng_calls, self.calculations, self.original_calls = 0, 0, 0
    self.rows = 0
    self.qword(self.menu + 0x30, self.parent)
    self.qword(self.menu + 0x38, 0)
    self.qword(self.original + 0x10, self.menu)
    self.results = self.list()
    for name in ("Master Seal", "Iron Sword", "Master Seal"):
      if name == "Master Seal" and self.values(self.results): item = self.values(self.results)[0]
      else:
        item = self.alloc()
        self.qword(item + 0x28, self.string("MID_" + name))
      self.append(self.results, item)
    before_class = bytes(self.cpu.mem_read(self.base_class, 0x400))
    self.call(replacement, self.parent, 0, 0, 0)
    assert self.original_calls == 1 and self.rows == 3
    values = self.values(self.full_list)
    assert len(values) == 3 and values[0] == self.original
    callbacks = []
    for item in values[1:]:
      klass = self.pointer(item)
      callbacks.append(self.pointer(klass + 0x138 + 18 * 16))
      for self.language in range(12):
        label = self.call(self.pointer(klass + 0x138 + 4 * 16), item, 0)
        assert self.read_string(label)
    self.call(replacement, self.parent, 0, 0, 0)
    assert self.values(self.full_list) == values
    assert bytes(self.cpu.mem_read(self.base_class, 0x400)) == before_class
    for flag, level in [(1, 1), (1, 5), (2, 3), (0, 5), (3, 5), (-1, 5), (1, 0), (1, 6)]:
      self.flag, self.level = flag, level
      for item, callback in zip(values[1:], callbacks):
        self.qword(self.menu + 0x38, 0)
        self.messages, self.seeds = [], []
        old_seed, rng_before, calc_before = self.seed, self.live_rng_calls, self.calculations
        result = self.call(callback, item, 0)
        eligible = flag in (1, 2) and 1 <= level <= 5
        reroll = item == values[2]
        assert result == (128 if eligible else 2048), (flag, level, reroll)
        assert self.live_rng_calls - rng_before == int(eligible and reroll)
        assert self.calculations - calc_before == int(eligible)
        if eligible:
          assert self.messages == ["Master Seal ×2\nIron Sword ×1"]
          assert len(self.seeds) == int(reroll)
        else: assert not self.messages and self.seed == old_seed
    self.flag, self.level, self.fail_calc = 1, 5, True
    self.qword(self.menu + 0x38, 0)
    old_seed = self.seed
    assert self.call(callbacks[1], values[2], 0) == 2048
    assert self.seed == old_seed
    self.active = False
