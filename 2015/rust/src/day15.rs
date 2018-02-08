extern crate nom;

#[derive(Debug)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32
}

fn parse_ingredient(raw_ingredient: &str) -> Ingredient {
    use self::nom::*;

    use std::str::FromStr;
    use std::str::from_utf8;

    named!(number<i32>, do_parse!(
        sign: opt!(tag_s!("-")) >>
        n: map!(digit, |d| -> i32 {
            FromStr::from_str(from_utf8(d).unwrap()).unwrap()
        }) >>
        (if sign.is_some() { -n } else { n })
    ));
    named!(name<String>, map!(
        take_while1!(is_alphabetic),
        |s| from_utf8(s).unwrap().to_string()
    ));
    named!(ingredient<Ingredient>, do_parse!(
        name: name >>
        tag_s!(": capacity ") >>
        capacity: number >>
        tag_s!(", durability ") >>
        durability: number >>
        tag_s!(", flavor ") >>
        flavor: number >>
        tag_s!(", texture ") >>
        texture: number >>
        tag_s!(", calories ") >>
        calories: number >>
        (Ingredient {
            name: name,
            capacity: capacity,
            durability: durability,
            flavor: flavor,
            texture: texture,
            calories: calories,
        })
    ));

    match ingredient(raw_ingredient.as_bytes()) {
        IResult::Done(_, entry) => entry,
        _                       => panic!("Wrong ingredient format")
    }
}

type Recipe<'a> = Vec<(&'a Ingredient, i32)>;

fn recipe_score(recipe: &Recipe) -> i32 {
    use std::cmp::max;

    recipe.iter().map(|&(i, n)| {
        [n * i.capacity, n * i.durability, n * i.flavor, n * i.texture]
    }).fold([0; 4], |acc, x|
        [acc[0] + x[0], acc[1] + x[1], acc[2] + x[2], acc[3] + x[3]]
    ).iter().fold(1, |acc, x| acc * max(0, *x))
}

fn distrib(sum: i32, count: usize) -> Vec<Vec<i32>> {
    if count == 1 {
        vec![vec![sum]]
    }
    else {
        (0..=sum).flat_map(|i| {
            let mut sub_distribs = distrib(sum - i, count - 1);
            sub_distribs.iter_mut().for_each(|d| d.push(i));
            sub_distribs
        }).collect()
    }
}

fn possible_recipes(ingredients: &Vec<Ingredient>) -> Vec<Recipe> {
    let distributions = distrib(100, ingredients.len());
    distributions.iter()
                 .map(|d| ingredients.iter().zip(d.iter())
                                            .map(|(i, d)| (i, *d)).collect())
                 .collect()

}

fn recipe_calories(recipe: &Recipe) -> i32 {
    recipe.iter().map(|&(i, n)| {
        i.calories * n
    }).fold(0, |acc, x| acc + x)
}

pub fn p1(input: &str) -> i32 {
    let ingredients = input.trim().split('\n')
                                  .map(parse_ingredient)
                                  .collect::<Vec<_>>();

    let all_recipes = possible_recipes(&ingredients);
    all_recipes.iter().map(recipe_score).max().unwrap_or(0)
}

pub fn p2(input: &str) -> i32 {
    let ingredients = input.trim().split('\n')
                                  .map(parse_ingredient)
                                  .collect::<Vec<_>>();

    let all_recipes = possible_recipes(&ingredients);
    all_recipes.iter().filter(|r| recipe_calories(r) == 500)
                      .map(recipe_score)
                      .max().unwrap_or(0)
}
