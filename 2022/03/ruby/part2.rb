lines = []
while line = gets
  lines << line.chomp
end

groups = lines.each_slice(3).to_a


commons = groups.map do |group|
  common = group.map(&:chars).inject(:&)[0]
  case common
  when 'a'..'z'
    common.ord - 'a'.ord + 1
  when 'A'..'Z'
    common.ord - 'A'.ord + 27
  end
end

puts commons.sum
