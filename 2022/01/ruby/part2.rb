counts = []
while line = gets
  counts << line
end
elves = counts.chunk{|l| l.chomp.empty? ? :_separator : true}

count_by_elf = elves.map do |_, elf|
  elf.map(&:to_i).sum
end

puts count_by_elf.max(3).sum

