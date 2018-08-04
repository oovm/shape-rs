use crate::Point;
use wolfram_library_link::expr::Expr;

pub trait ToWolfram {
    fn to_wolfram(&self) -> Expr;
}

impl<T> ToWolfram for Point<T>
where
    Expr: From<T>,
{
    fn to_wolfram(&self) -> Expr {
        let x = Expr::from(self.x);
        let y = Expr::from(self.y);

        let list = Expr::list(vec![x, y]);
        Expr::normal(Expr::symbol("Point"), list)
    }
}

impl<T> ToWolfram for Point3D<T>
where
    Expr: From<T>,
{
    fn to_wolfram(&self) -> Expr {
        let x = Expr::from(self.x);
        let y = Expr::from(self.y);
        let z = Expr::from(self.z);

        let list = Expr::list(vec![x, y, z]);
        Expr::normal(Expr::symbol("Point"), list)
    }
}
