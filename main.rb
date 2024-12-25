require './lexer'

class MacroProcessor
  def initialize tokens
    @tokens = tokens
    @macros = {}
  end
  attr_reader :macros

  def run
    result = []
    @i = 0
    while @i < @tokens.size
      line = @tokens[@i]
      @i += 1
      case line[0]
      when /!\w+/
        define_macro *line
      when /\w+!/
        result.push *use_macro(line)
      else
        result << line
      end
    end
    result.map { |s| s.join' ' }.join"\n"
  end

  def define_macro name, *args
    name = name[1..]
    @macros[name] = {args: args, body: []}
    until @tokens[@i][0] == '!!'
      @macros[name][:body] << @tokens[@i]
      @i += 1
    end
    @i += 1
  end

  def use_macro line
    name = line[0][..-2]
    args = pp @macros[name][:args].zip(line[1..]).to_h
    body = @macros[name][:body]

    body.map do |line|
      line.map do |token|
        if /^\w+!$/ =~ token && args[token[..-2]]
          args[token[..-2]]
        else
          token
        end
      end
    end
  end
end

# source = 'set i 0.0
# op add i i 1
# print "Hi!"'

source = File.read './example.mll'

tokens = Lexer.new(source).all_lines
processor = MacroProcessor.new(tokens)
result = puts processor.run
pp processor.macros

