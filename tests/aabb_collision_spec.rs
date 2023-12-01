#![cfg(feature = "aabb")]

use rstest::rstest;

use collision2d::{Aabb, Collides, Penetration};

#[rstest]
#[case(Aabb::from_min_max([0., 0.], [1., 1.]), Aabb::from_min_max([2., 0.], [3., 1.]))]
#[case(Aabb::from_min_max([0., 0.], [1., 1.]), Aabb::from_min_max([1., 0.], [2., 1.]))]
#[case(Aabb::from_min_max([0., 0.], [1., 1.]), Aabb::from_min_max([-2., 0.], [-1., 1.]))]
#[case(Aabb::from_min_max([0., 0.], [1., 1.]), Aabb::from_min_max([0., -2.], [0., -1.]))]
#[case(Aabb::from_min_max([0., 0.], [1., 1.]), Aabb::from_min_max([0., 2.], [0., 3.]))]
fn should_not_collide(#[case] shape1: Aabb, #[case] shape2: Aabb) {
    assert!(!shape1.collides(&shape2));
    assert!(!shape1.collides_any([&shape2]));
    assert_eq!(shape1.penetration(&shape2), None);
    assert_eq!(shape1.max_penetration([&shape2]), None);
}

#[test]
fn should_collide_with_self() {
    let shape = Aabb::from_min_max([0., 0.], [1., 1.]);
    assert!(shape.collides(&shape));
    let [x, y] = shape.penetration(&shape).unwrap();
    assert!((x.abs() + y.abs() - 1.).abs() < f32::EPSILON);
}

#[rstest]
#[case(Aabb::from_min_max([0., 0.], [1., 1.]), Aabb::from_min_max([0.5, 0.], [1.5, 1.]), [-0.5, 0.0])]
#[case(Aabb::from_min_max([0., 0.], [1., 1.]), Aabb::from_min_max([-0.5, 0.], [0.5, 1.]), [0.5, 0.0])]
#[case(Aabb::from_min_max([0., 0.], [1., 1.]), Aabb::from_min_max([0., -0.5], [1.0, 0.5]), [0.0, 0.5])]
#[case(Aabb::from_min_max([0., 0.], [1., 1.]), Aabb::from_min_max([0., 0.5], [1.0, 1.5]), [0.0, -0.5])]
fn should_collide(
    #[case] shape1: Aabb,
    #[case] shape2: Aabb,
    #[case] expected_penetration: [f32; 2],
) {
    println!("shape1: {shape1:?}");
    println!("shape2: {shape2:?}");
    assert!(shape1.collides(&shape2));
    assert!(shape1.collides_any([&shape2]));
    let actual_penetration = shape1
        .penetration(&shape2)
        .expect("the shape are not penetrating");
    assert!(
        (actual_penetration[0] - expected_penetration[0]).abs() <= f32::EPSILON,
        "{actual_penetration:?} != {expected_penetration:?}"
    );
    assert!(
        (actual_penetration[1] - expected_penetration[1]).abs() <= f32::EPSILON,
        "{actual_penetration:?} != {expected_penetration:?}"
    );
}

#[rstest]
fn collides_any_returns_false_for_empty_iter() {
    assert!(!Aabb::from_min_max([0., 0.], [1., 1.]).collides_any([]));
}
