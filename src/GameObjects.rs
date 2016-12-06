mod GameObjects {
	struct Ball{
		xpos: u32,
		ypos: u32,
		radius: u32,
		speed: u32,
		direction: [f32; 2],
	}
	impl Ball{
		fn change_dir(&mut self, dir: u32) {
			if dir == 1 {
				self.direction[0] *= -1;
			} else {
				self.direction[1] *= -1;
			}
		}
		fn next_step(&mut self) {
			self.xpos += (self.direction[0] * self.speed);
			self.ypos += (self.direction[1] * self.speed);
		}
		fn new_pos(&mut self, x: u32, y: u32) {
			self.xpos = x;
			self.ypos = y;
		}
		fn get_posx(&self) -> u32 {
			self.xpos
		}
		fn get_posx(&self) -> u32 {
			self.xpos
		}
	}
	
	struct Player{
		xpos: u32,
		ypos: u32,
		length: u32,
	}
	
	impl Player {
		fn move_up(&mut self, pixels: u32) {
			self.ypos+= pixels;
		}
		fn move_down(&mut self, pixels: u32) {
			self.ypos-= pixels;
		}
		fn get_xpos(&self) -> u32 {
			self.xpos
		}
		fn get_ypos(&self) -> u32 {
			self.ypos
		}
		fn increase_len(&self, len: uint){
			self.length += len;
		}
		fn decrease_len(&self, len: uint){
			self.length -= len;
		}
	}
}