#[macro_export]
macro_rules! emit {
	(Quit($opt:expr)) => {
		yazi_shared::event::Event::Quit($opt).emit();
	};
	(Call($cmd:expr, $layer:expr)) => {
		yazi_shared::event::Event::Call(yazi_shared::event::CmdCow::from($cmd), $layer).emit();
	};
	(Seq($cmds:expr, $layer:expr)) => {
		yazi_shared::event::Event::Seq($cmds, $layer).emit();
	};
	($event:ident) => {
		yazi_shared::event::Event::$event.emit();
	};
}

#[macro_export]
macro_rules! render {
	() => {
		yazi_shared::event::NEED_RENDER.store(true, std::sync::atomic::Ordering::Relaxed);
	};
	($cond:expr) => {
		if $cond {
			render!();
		}
	};
}

#[macro_export]
macro_rules! render_and {
	($cond:expr) => {
		if $cond {
			yazi_macro::render!();
			true
		} else {
			false
		}
	};
}