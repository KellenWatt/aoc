code = gets.chomp
offset = code.chars.each_cons(14).take_while do |chars|
  puts chars.uniq.to_s
  chars.uniq.size != 14
end
puts offset.size
