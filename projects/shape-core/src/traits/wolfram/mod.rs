use crate::{Point, Point3D};
use wolfram_expr::{Expr, Symbol};

impl<T> Point<T>
where
    Expr: From<T>,
    T: Clone,
{
    pub fn as_wolfram(&self) -> Expr {
        let x = Expr::from(self.x.clone());
        let y = Expr::from(self.y.clone());
        let list = Expr::list(vec![x, y]);
        Expr::normal(Symbol::new("System`Point"), vec![list])
    }
}

impl<T> Point3D<T>
where
    Expr: From<T>,
    T: Clone,
{
    pub fn as_wolfram(&self) -> Expr {
        let x = Expr::from(self.x.clone());
        let y = Expr::from(self.y.clone());
        let z = Expr::from(self.z.clone());
        let list = Expr::list(vec![x, y, z]);
        Expr::normal(Symbol::new("System`Point"), vec![list])
    }
}
