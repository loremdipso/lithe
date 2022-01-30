#!/usr/bin/env ruby

def main(args)
	Dir.chdir(File.join(__dir__, '..', 'files')) do
		total_compiled_size = 0
		total_minified_size = 0
		Dir['compiled/**'].each do |f|
			basename = File.basename(f)
			minified = "minified/#{basename}"
			if !File.exists?(minified)
				#puts "#{f} isn't minified"
			else
				compiled_size = File.size(f)
				minified_size = File.size(minified)

				total_compiled_size += compiled_size
				total_minified_size += minified_size

				if compiled_size < minified_size
					puts "\nFor some reason, the minified size is bigger"
					puts "#{f} => #{compiled_size}"
					puts "#{minified} => #{minified_size}"
				end
			end
		end

		puts "Total compiled: #{total_compiled_size}"
		puts "Total minified: #{total_minified_size}"
	end
end


main(ARGV.dup)

