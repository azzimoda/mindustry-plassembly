class Token
  def initialize type, value, line:
    @type = type
    @value = value
    @line
  end
  attr_reader :type, :value
end

class Lexer
  TOKEN_RE= /!!|-?\d+\.\d+|-?\d+|[@!]\w+|\w+[:!]?|"([^"\\\n]|\\"|\\)*?"/m

  def initialize input
    @input = input
    @lines = @input.lines.map(&:strip).select { |s| !s.empty? }
    @line = 0
  end

  def all_lines
    lines = []
    while @line < @lines.size
      lines << next_line
    end
    lines.select { |e| !e.empty? }
  end

  def next_line
    tokens = @lines[@line].to_enum(:scan, TOKEN_RE).map { $& }
    @line += 1
    tokens
  end

  # def next_token
  #   return :eof if @i >= @input.size
    
  #   skip_space

  #   s = @input[@i..]
  #   type = if /^-?\d+\.\d+\b/ =~ s
  #     :float
  #   elsif /^-?\d+\b/ =~ s
  #     :int
  #   elsif /^\w+\b/ =~ s
  #     :word
  #   elsif /^"([^"]|\\")"/ =~ s
  #     :string
  #   elsif /^@\w+\b/ =~ s
  #     :sysvar
  #   else
  #     :unknown
  #   end
  #   return nil if :unknown

  #   @i += $~.end 0
  #   Token.new type, $~.match(0), line:
  # end

  def skip_space
    i += /\s*/ =~ @input[i..]
    @line += $~.match(0).count("\n")
  end
end


