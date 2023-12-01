
def is_number_word(word)
  word.match(/^(one|t(wo|hree)|f(our|ive)|s(ix|even)|eight|nine)/)&.match_length(0)
end

def word_value(word)
  case word
  when "one" then 1
  when "two" then 2
  when "three" then 3
  when "four" then 4
  when "five" then 5
  when "six" then 6
  when "seven" then 7
  when "eight" then 8
  when "nine" then 9
  else nil
  end
end

def line_to_digits(line)
  digits = []
  while line.size > 0
    if line[0].to_i.to_s == line[0]
      digits << line[0].to_i
    else 
      len = is_number_word(line)
      unless len.nil?
        word = line[...len]
        digits << word_value(word)
      end
    end
    line = line[1..]
  end
  digits
end

lines = []
while line = gets&.chomp
  lines << line
end

nums = lines.map do |line|
  digits = line_to_digits(line)
  first = digits[0]
  last = digits[-1]
  num = first * 10 + last
  # puts "#{line}: #{digits} = #{num}"
  num
end

puts nums.sum()

