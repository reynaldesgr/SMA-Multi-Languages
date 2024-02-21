class Position:
    def __init__(self, ix=0, iy=0):
        """Initializes the position with specified x and y coordinates or defaults to (0, 0)."""
        self.x = ix
        self.y = iy

    def get_x(self):
        """Returns the x-coordinate."""
        return self.x

    def get_y(self):
        """Returns the y-coordinate."""
        return self.y

    def set_x(self, new_x):
        """Sets the x-coordinate to a new value."""
        self.x = new_x

    def set_y(self, new_y):
        """Sets the y-coordinate to a new value."""
        self.y = new_y

    def __repr__(self):
        """Returns a string representation of the position."""
        return f"(x={self.x}, y={self.y})"
