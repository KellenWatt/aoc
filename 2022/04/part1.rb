lines = []
while line = gets
  lines << line.chomp
end

pairs = lines.map{|l| l.split(",")}
pairs = pairs.map do |e, f|
  [Range.new(*(e.split("-"))), Range.new(*(f.split("-")))]
end

overlapping = pairs.inject(0) do |acc, pair|
  e, f = pair
  if e.all?{|el| f.include?(el)} || f.all?{|fl| e.include?(fl)}
    acc + 1
  else 
    acc
  end
end

puts overlapping
