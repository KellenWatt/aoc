grid = []
while line = gets
  grid << line.chomp.split("").map(&:to_i)
end

def scenic_score(grid, x, y)
  height = grid[y][x]
  right = grid[y][(x+1)..].reduce(0) do |acc, t|
    acc += 1
    if t >= height
      break acc
    end
    acc
  end
  left = grid[y][...x].reverse.reduce(0) do |acc, t|
    acc += 1
    if t >= height
      break acc
    end
    acc
  end
  top = grid[...y].reverse.reduce(0) do |acc, col|
    acc += 1
    if col[x] >= height
      break acc
    end
    acc
  end
  bottom = grid[(y+1)..].reduce(0) do |acc, col|
    acc += 1
    if col[x] >= height
      break acc
    end
    acc
  end
 
  puts "(#{x},#{y}): #{[right, left, top, bottom]}"

  right * left * top * bottom
end


score = grid.each_with_index.reduce(0) do |acc, ewi|
  row, y = ewi
  row_score = row.map.with_index do |tree, x|
    scenic_score(grid, x, y)
  end.max
  row_score > acc ? row_score : acc
end

puts score
