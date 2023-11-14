lines = []
while line = gets
  lines << line.chomp
end

crates_text = lines.take_while{|l| !l.empty?}
lines.shift(crates_text.size + 1)
crates_text.pop

crates = crates_text.map do |line|
  line.chars.each_slice(4).map do |crate|
    crate = crate.join
    if crate =~ /\[(.)\]/
      Regexp.last_match[1]
    end
  end
end.reverse.transpose

crates.map!(&:compact)

lines.each do |instr|
  match = instr.match(/move (?<count>\d+) from (?<from>\d) to (?<to>\d)/)
  from = match[:from].to_i - 1
  to = match[:to].to_i - 1
  count = match[:count].to_i

  cs = crates[from].pop(count)
  crates[to].push(*cs)
end

puts crates.map(&:last).join
