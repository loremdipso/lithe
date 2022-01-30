#!/usr/bin/env ruby
# Usage: ./generate_svelte_file.rb #{i}

def main(args)
	Dir.chdir(File.join(__dir__, '..', 'test')) do
		num_lines = args[0].to_i
		if num_lines.to_s != args[0]
			puts "ERROR: #{args[0]} invalid number"
			return
		end

		puts "Generating %d lines" % num_lines
		line = "<span>Hello world!</span>"
		File.open("test.svelte", "w") do |f|
			num_lines.times do
				f.print(line)
			end
		end
	end
end

main(ARGV.dup)

