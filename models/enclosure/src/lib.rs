use std::collections::HashMap;

#[no_mangle]
pub extern "C" fn model(_args: &HashMap<String, String>) -> fj::Shape {
    // # Prusa Mini Enclosure
    //
    // A friend offered to build an enclosure for my Prusa Mini out of plywood,
    // as long as I design it. This file presents the design and documents the
    // design intent. It is intentionally vague in some areas, as many decisions
    // are left up to my friend.
    //
    // All units are in millimeters.
    //
    //
    // ## Internal Dimensions
    //
    // Let's take a look at the internal dimensions of the enclosure, how I came
    // up with them, and the constraints they are driven by. The design intent
    // here is to define dimensions that fit the printer, allow me to handle the
    // printer, but are otherwise as small as practical, to not make the
    // enclosure overly large.
    //
    // ### Width
    //
    // The approximate width of the printer:
    let printer_width = 325.;

    // We need some extra wiggle room to take the printer into or out of the
    // enclosure. Here's a nice value for the left side:
    let margin_left = 30.;

    // The right side needs a larger margin. You'd typically lift the printer by
    // grasping the Z axis extrusion from the right. The following margin should
    // allow me to do that comfortably:
    let margin_right = 60.;

    // The final width is the sum of those numbers:
    let inner_width = printer_width + margin_left + margin_right;

    // ### Depth
    //
    // Measuring the depth of the printer is complicated by the fact that the Y
    // axis is moving front-to-back.
    //
    // Let's start with the length of the Y axis assembly's base:
    let y_assembly_base_depth = 285.;

    // If the Y axis is in its front-most position, it overhangs this much on
    // the front:
    let print_bed_overhang_front = 55.;

    // With a bit of additional margin, this results in the front margin:
    let margin_front = print_bed_overhang_front + 20.;

    // If the Y axis is in its back-most position, it overhangs this much on the
    // back:
    let print_bed_overhang_back = 50.;

    // We can ignore the electronics enclosure. It protrudes behind the Y axis
    // base, but is completely covered by the back overhang.
    //
    // In addition to the overhang, we need to consider the cable going to the
    // heated bed. This should provide enough clearance for the plug and the
    // cable, without bending it too much:
    let margin_heat_bed_cable = 60.;

    // Together, this results in the back margin:
    let margin_back = print_bed_overhang_back + margin_heat_bed_cable;

    // Inner depth is the sum of all of these:
    let inner_depth = y_assembly_base_depth + margin_front + margin_back;

    // ### Height
    //
    // Now the height. This one is the most straight-forward. First, the printer
    // height:
    let printer_height = 385.;

    // Next, a bit of margin on top to take it into or out of the enclosure:
    let margin_top = 30.;

    // Sum it up to get the total height:
    let inner_height = printer_height + margin_top;

    // These are the values for the inner dimensions:
    assert_eq!(inner_width, 415.);
    assert_eq!(inner_depth, 470.);
    assert_eq!(inner_height, 415.);

    // ## Outer Dimensions
    //
    // To compute the outer dimensions, we need to know the material strength.
    // The following is my current assumption:
    let material_strength = 12.;

    // This gives us the following outer dimensions:
    let outer_width = inner_width + material_strength * 2.;
    let outer_depth = inner_depth + material_strength * 2.;
    let outer_height = inner_height + material_strength * 2.;

    // These are the values for the outer dimensions:
    assert_eq!(outer_width, 439.);
    assert_eq!(outer_depth, 494.);
    assert_eq!(outer_height, 439.);

    // TASK: Implement.
    todo!()
}
