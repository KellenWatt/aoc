class Directory
  include Enumerable
  attr_reader :files, :name, :subdirs, :parent
  def initialize(name, parent = nil)
    @files = {}
    @name = name
    @parent = parent
    @subdirs = {}
  end

  def add_file(name, size)
    @files[name] = size
  end

  def add_directory(name)
    @subdirs[name] = Directory.new(name, self)
  end

  def size
    @files.values.sum + @subdirs.values.map(&:size).sum
  end

  def each(&block)
    if !block_given?
      return to_enum(:each)
    end

    yield self
    @subdirs.values.each do |dir|
      dir.each(&block)
    end
  end
end

def parse_ls(lines, cwd)
  lines.each do |line|
    case line
    when /^dir (?<dirname>.+)$/
      cwd.add_directory(Regexp.last_match[:dirname])
    when /^(?<size>\d+) (?<filename>.+)$/
      match = Regexp.last_match
      cwd.add_file(match[:filename], match[:size].to_i)
    end
  end
end

def do_cd(line, cwd)
  name = (line.match(/cd (?<name>.+)/))[:name]
  if name == ".."
    cwd.parent
  else
    cwd.subdirs[name]
  end
end

lines = []
while line = gets 
  lines << line.chomp
end

tree = Directory.new("/")
cwd = tree

i = 1
while i < lines.size
  case lines[i]
  when /^\$ cd/
    cwd = do_cd(lines[i], cwd)
    i += 1
  when /^\$ ls/
    output = lines[(i+1)..].take_while{|l| !l.start_with?("$")}
    parse_ls(output, cwd)
    i += output.size + 1
  end
end

## Setup complete

size = tree.size
free_space = 7e7 - size
target = 3e7
to_free = target - free_space

min_dir = tree.min_by do |dir|
  diff = dir.size - to_free
  if diff < 0
    Float::INFINITY
  else
    diff
  end
end

puts min_dir.size


