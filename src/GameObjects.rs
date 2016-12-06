mod GameObjects {
	/*struct Ball{
		xpos: u32,
		ypos: u32,
		radius: u32,
		speed: u32,
		direction: [i32; 2],
	}
	impl Ball{
		fn next_step() {
		
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
	*/
	
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
			self.ypos-= [pixels;
		}
		fn get_xpos(&self) -> u32 {
			self.xpos
		}
		fn get_ypos(&self) -> u32 {
			self.ypos
		}
	}