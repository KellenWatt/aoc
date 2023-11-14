require "set"

grid = []
while line = gets
  grid << line.chomp.split("").map(&:to_i)
end


seen = Set.new

grid.each_with_index do |row, y|
  left = -1
  row.each_with_index do |tree, x|
    if tree > left
      seen << [x, y]
      left = tree
    end
  end
  right = -1
  row.each_with_index.reverse_each do |ewi|
    tree, x = ewi
    if tree > right
      seen << [x, y]
      right = tree
    end
  end
end

x, y = 0, 0
while x < grid[0].size
  top = -1
  while y < grid.size
    tree = grid[y][x]
    if tree > top
      seen << [x, y]
      top = tree
    end
    y += 1
  end

  bottom = -1
  while y > 0
    y -= 1
    tree = grid[y][x]
    if tree > bottom
      seen << [x, y]
      bottom = tree
    end
  end
  x += 1
end

puts seen.size


