pos = 0
depth = 0
while line = gets
  case line
  when /forward (?<position>\d+)/ 
    pos += Regexp.last_match[:position].to_i
  when /down (?<depth>\d+)/ 
    depth += Regexp.last_match[:depth].to_i
  when /up (?<depth>\d+)/
    depth -= Regexp.last_match[:depth].to_i
  end
end
puts pos * depth
