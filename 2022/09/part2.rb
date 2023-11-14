require "set"

def distance(head, tail)
  Math.sqrt((head[0] - tail[0])**2 + (head[1] - tail[1])**2)
end

def update_head(head_pos, dir)
  case dir
  when "U" then [head_pos[0], head_pos[1]+1]
  when "D" then [head_pos[0], head_pos[1]-1]
  when "R" then [head_pos[0]+1, head_pos[1]]
  when "L" then [head_pos[0]-1, head_pos[1]]
  end
end

def update_tail(head, tail)
  if distance(head, tail) < 2
    return tail
  end

  x, y = tail
  if head[0] > x
    x += 1
  elsif head[0] < x
    x -= 1
  end

  if head[1] > y
    y += 1
  elsif head[1] < y
    y -= 1
  end
  [x, y]
end


lines = []
while line = gets
  lines << line.chomp
end

knots = 10.times.map{[0,0]}

seen = Set.new

directions = lines.map do |l|
  dir, n = l.split
  n = n.to_i
  [dir, n]
end

directions.each do |dir, n|
  n.times do 
    seen << knots.last
    knots[0] = update_head(knots[0], dir)
    i = 1
    while i < knots.size
      knots[i] = update_tail(knots[i-1], knots[i])
      i += 1
    end
  end
end

puts "-----"
puts seen.size


