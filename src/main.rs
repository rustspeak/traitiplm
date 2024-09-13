trait Figureform {
    fn perimetre(x :  i32, y: i32) -> i32 ;
    fn aire(x:i32, y: i32) -> i32 ;

}
enum Figure  {
    carre(i32),
    rectangle(i32, i32),
    triangle(i32, i32),
    trapeze(i32 ,  i32 , i32),
    cube(i32),

}

impl   Figure {
    fn  name(&self) -> &'static str {
    match  self {
        Figure::carre(_) => "carre",
        Figure::rectangle(_,_ ) => "  rectangle",
        Figure::triangle(_,_) => "triangle",
        Figure::trapeze(_,_,_) => "  trapeze",
        Figure::cube(_) => "cube",
        _ => " autre figure  non  mentionner  dans  les structure ",
    }
}
    fn  perimetre(&self ) -> i32 {
        match self {
            Figure::carre(c ) => c*c,
            Figure::rectangle(L, l ) => l*L,
            Figure::triangle(b,h ) => (b * h) / 2,
            Figure::trapeze(M , m, h) => ((m + M) * h) / 2 ,
            Figure::cube(c) => (c * c) * 6 ,
        }
    }

}




fn main() {
    let    carre =  Figure::carre(23);
    let    result = carre.name();
    let   resutl2 = carre.perimetre();
    let  cube = Figure::cube(5);
    let  resultcube = cube.name();
    let resultcube2 = cube.perimetre();
    println!(" le  nom  est  {:?} et le  perimetre est : {:?}" , result, resutl2);
    println!(" le  nom de figure  est {:?} et  sont  perimetre est : {:?} ", resultcube, resultcube2);

 }
