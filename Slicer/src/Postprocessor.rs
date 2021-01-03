use crate::Translator::QuadrantBlock;
use crate::TranslatorUtil;
use crate::GCode;

pub fn postprocess(blocks: Vec<QuadrantBlock>) -> Vec<GCode::GCode> {
    let processed_blocks = group_and_reduce_gcode(blocks);

    let mut gcode: Vec<GCode::GCode> = Vec::new();

    let mut current_quadrant: (i32, i32) = processed_blocks[0].quadrant;
    gcode.push(TranslatorUtil::point_to_move_quadrant_cmd(current_quadrant));

    for block in processed_blocks {
        if block.quadrant != current_quadrant {
            gcode.push(TranslatorUtil::point_to_move_quadrant_cmd(block.quadrant));
        }

        for code in block.gcode {
            gcode.push(code);
        }

        gcode.push(GCode::GCode {
            command: GCode::Word {
                letter: 'G',
                value: 1
            },
            x: 0.0,
            y: 0.0,
            z: -1.0
        });

        current_quadrant = block.quadrant;
    }

    gcode
}

pub fn group_and_reduce_gcode(blocks: Vec<QuadrantBlock>) -> Vec<QuadrantBlock>{

    let mut quadrant_list: Vec<(i32, i32)> = Vec::new();
    // TODO: Alot of this work could be cut down by having better data structures - Austin Haskell
    for block in &blocks {
        if quadrant_list.contains(&block.quadrant) {
            continue;
        }

        quadrant_list.push(block.quadrant);
    }

    // Sort by quadrant
    let mut grouped_blocks: Vec<QuadrantBlock> = Vec::new();
    for quadrant in quadrant_list {

        // TODO: Make this not suck - Austin Haskell
        for block in &blocks {
            if block.quadrant == quadrant {
                grouped_blocks.push(block.clone());
            }
        }
    }

    grouped_blocks
}