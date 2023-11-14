class Emulator
  @@instruction_set = {}
  def self.register_instruction(name, time, &block)
    @@instruction_set[name] = InstructionDefinition.new(name, 0, time, &block)
  end

  attr_reader :time
  def initialize(instructions = [])
    @instructions = load_program(instructions)
    reset
  end

  def reset
    @time = 0
    @pointer = 0
    @registers = {
      X: 1
    }
    @time_remaining = @instructions.empty? ? 0 : current_instruction&.definition&.time_required
  end

  def load_program(instructions)
    @instructions = instructions.map do |inst| 
      name, *args = inst.split
      Instruction.new(@@instruction_set[name], *args)
    end
  end

  def step!
    raise InstructionError, "No instruction to read" unless program_loaded?
    @time += 1
    return if program_complete?
    @time_remaining -= 1
    if @time_remaining == 0
      current_instruction.call(@registers)
      @pointer += 1
      @time_remaining = current_instruction.definition.time_required unless program_complete?
    end
  end

  def step_to!(n)
    raise ArgumentError, "Can't step backwards" if n < @time
    step! until @time == n
  end

  def registers
    @registers.dup
  end


  def program_loaded?
    !@instructions.empty?
  end

  def program_complete?
    @pointer >= @instructions.size
  end

  def duration
    @instructions.map{|i| i.definition.time_required}.sum
  end

  class InstructionError < StandardError
  end

  class InstructionDefinition
    attr_reader :name, :required_args, :effect, :time_required
    def initialize(name, argc, time, &block)
      raise ArgumentError, "Instruction requires associated behaviour" unless block_given?
      @name = name
      @required_args = argc
      @time_required = time
      @effect = block
    end
  end

  private
  class Instruction
    attr_reader :definition
    attr_accessor :args
    def initialize(definition, *args)
      raise ArgumentError, "Unrecognized definition" if definition.nil?
      @args = args.map(&:to_i)
      @definition = definition
    end

    def call(registers)
      @definition.effect.call(registers, *args)
    end
  end

  def current_instruction
    @instructions[@pointer]
  end
end

Emulator.register_instruction("noop", 1){}
Emulator.register_instruction("addx", 2){|reg, n| reg[:X] += n}


lines = []
while line = gets
  lines << line.chomp
end

e = Emulator.new(lines)


puts e.duration
monitor = ["."] * 240
crt = 0
until e.program_complete?
  x = e.registers[:X]
  if crt % 40 == x % 40 || crt % 40 == (x-1)%40 || crt % 40 == (x+1)%40
    monitor[crt] = "#"
    puts crt
  end
  puts "#{e.time}: #{e.registers}"
  crt = (crt+1)%240
  e.step!
end

monitor.each_slice(40) do |row|
  puts row.join("")
end






