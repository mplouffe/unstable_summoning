use crate::prelude::*;

#[system]
#[write_component(Render)]
#[write_component(Animation)]
pub fn animation(
    ecs: &mut SubWorld,
    #[resource] timer_info: &TimerInfo,
    #[resource] animations: &Vec<Vec<usize>>
) {
    let mut animateable = <(&mut Render, &mut Animation)>::query();

    animateable
        .iter_mut(ecs)
        .for_each(|(render, animation)| {
            let current_frame;
            let frames = &animations[animation.animation_index];
            match animation.state {
                AnimationState::Start => {
                    animation.starting_frame = timer_info.frame;
                    animation.state = AnimationState::Playing;
                    render.index = frames[0];
                },
                AnimationState::Playing => {
                    let playing_delta = timer_info.frame - animation.starting_frame;
                    let animation_length = frames.len();

                    if !animation.loop_play && playing_delta > animation_length
                    {
                        animation.state = AnimationState::Stopping;
                    }
                    else
                    {
                        current_frame =  playing_delta % animation_length;
                        render.index = frames[current_frame];
                    }
                },
                AnimationState::Stopping => {
                    animation.state = AnimationState::Stopped;
                    render.index = frames[animation.final_frame];
                },
                AnimationState::Stopped => { }
            }
        });
}