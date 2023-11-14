pos = 0
aim = 0
depth = 0

while line = gets
  case line
  when /forward (?<position>\d+)/ 
    n = Regexp.last_match[:position].to_i
    pos += n
    depth += n * aim

  when /down (?<depth>\d+)/ 
    aim += Regexp.last_match[:depth].to_i
  when /up (?<depth>\d+)/
    aim -= Regexp.last_match[:depth].to_i
  end
end
puts pos * depth
