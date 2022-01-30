#!/usr/bin/env ruby

def main(args)
	Dir.chdir(File.join(__dir__, '..')) do
		shuffled = Dir[File.join('test', 'files', 'cleaned', '*.svelte')].shuffle

		filename = shuffled[0]
		File.open(File.join("test", "test.svelte"), "w") do |f|
			contents = File.read(filename)

			p filename
			puts contents
			f.puts("<!-- #{filename} -->")
			f.print(contents)
		end
	end
end

main(ARGV.dup)

