#!/usr/bin/env ruby
require 'fileutils'

def main(args)
	Dir.chdir(File.join(__dir__, '..', 'files')) do
		case args[0]
		when "all"
			extract()
			dedup()
			compile(false)
			minify()
		when "extract"
			extract()
		when "gzip"
			gzip()
		when "dedup"
			dedup()
		when "compile"
			compile(false)
		when "compile_and_minify"
			compile(true)
		when "minify"
			minify()
		else
			usage()
		end
	end
end

def usage()
	puts "#{__FILE__} all|extract|dedup|compile|minify|compile_and_minify|gzip"
	puts "NOTE: this is pretty slow. Might want to make yourself a cuppa :)"
end

def compile(do_minify)
	if do_minify
		puts "Compiling and minifying..."
	else
		puts "Compiling raw..."
	end

	cleaned_dir = get_cleaned_dir()
	compiled_dir = get_compiled_dir(do_minify)
	files = Dir["#{cleaned_dir}/**"]
	files.each_with_index do |f,i|
		puts "#{i+1} / #{files.size}"
		output = `node ../test.mjs --only_js --show_output #{do_minify ? "--minify" : ""} --filename "#{f}"`
		if output.start_with?("ERROR:")
			puts "Error: skipping"
		else
			File.open("#{compiled_dir}/#{File.basename(f, File.extname(f))}.js", "w") {|f| f.print(output)}
		end
	end
end

def minify()
	puts "Minifying..."

	compiled_dir = get_compiled_dir(false)
	minify_dir = get_compiled_dir(true)
	files = Dir["#{compiled_dir}/**"]
	files.each_with_index do |f,i|
		puts "#{i+1} / #{files.size}"
		output = `node ../minify.mjs --filename "#{f}"`
		if output.start_with?("ERROR:")
			puts "Error: skipping"
		else
			File.open("#{minify_dir}/#{File.basename(f, File.extname(f))}.js", "w") {|f| f.print(output)}
		end
	end
end

def gzip()
	puts "GZipping..."

	minify_dir = get_compiled_dir(true)
	gzip_dir = get_gzip_dir()
	files = Dir["#{minify_dir}/**"]
	files.each_with_index do |f,i|
		puts "#{i+1} / #{files.size}"
		output_file = File.join(gzip_dir, File.basename(f))
		`gzip -c "#{f}" > "#{output_file}"`
	end
end

def dedup()
	puts "Deduping..."
	cleaned_dir = get_cleaned_dir()
	sizes = {}

	files = Dir["#{cleaned_dir}/**/*.svelte"]
	files.each do |f|
		size = File.size(f)
		sizes[size] ||= []
		if sizes[size].size() > 0
			files = sizes[size]
			contents = File.read(f)
			had_duplicate = false
			files.each do |possible_duplicate|
				if File.read(possible_duplicate) == contents
					puts "Duplicate! #{f}"
					had_duplicate = true
					break
				end
			end

			if had_duplicate
				File.delete(f)
			else
				sizes[size].push(f)
			end
		else
			sizes[size].push(f)
		end
	end
end

def extract()
	puts "Extracting..."
	cleaned_dir = get_cleaned_dir()
	files = Dir['raw/**/*.svelte']
	files.each_with_index do |f,i|
		puts "#{i} / #{files.size}"
		FileUtils.cp(f, "#{cleaned_dir}/%04d.svelte" % i)
	end
end

def get_cleaned_dir()
	dir = "cleaned"

	if !File.exists?(dir)
		Dir.mkdir(dir)
	end

	return dir
end

def get_gzip_dir()
	dir = "gzip"
	if !File.exists?(dir)
		Dir.mkdir(dir)
	end
	return dir
end

def get_compiled_dir(do_minify)
	dir = "compiled"
	if do_minify
		dir = "minified"
	end

	if !File.exists?(dir)
		Dir.mkdir(dir)
	end

	return dir
end

main(ARGV.dup)

