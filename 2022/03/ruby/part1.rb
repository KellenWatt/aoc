lines = []
while line = gets
  lines << line
end
packs = lines.map{|l| [l[...(l.size/2)].chars.to_a, l[(l.size/2)..].chars.to_a]}

shared = packs.map do |p|
  p[0] & p[1]
end

puts shared.to_s

values = shared.map do |letters|
  letters.map do |letter|
    case letter
    when 'a'..'z' then letter.ord - 'a'.ord + 1
    when 'A'..'Z' then letter.ord - 'A'.ord + 27
    end
  end
end

puts values.to_s
puts values.map(&:sum).sum
