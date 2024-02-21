class Position
    attr_accessor :x, :y

    def initialize(ix = 0, iy = 0)
      @x = ix
      @y = iy
    end

    def self.default
      new
    end
  
    def get_x
      @x
    end
  
    def get_y
      @y
    end
  
    def set_x(new_x)
      @x = new_x
    end
  
    def set_y(new_y)
      @y = new_y
    end
  end  