code = gets.chomp
offset = code.chars.each_cons(4).take_while do |chars|
  puts chars.uniq.to_s
  chars.uniq.size != 4
end
puts offset.size
