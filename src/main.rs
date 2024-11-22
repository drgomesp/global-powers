use crate::population::{Class, Group, Profession, Religion};
use crate::region::{Country, State};

mod population;
mod region;

fn main() {
    let mut brazil = Country::new();

    let mut sc = State::new("SC".into(), "Santa Catarina".into());
    let mut rs = State::new("RS".into(), "Rio Grande do Sul".into());
    let mut sp = State::new("SP".into(), "Sao Paulo".into());

    let catholic = Religion::new("Catholic".into());
    let protestant = Religion::new("Protestant".into());
    let agnostic = Religion::new("Agnostic".into());

    let construction_worker = Profession::new(Class::Lower, "Construction Worker".into());
    let public_worker = Profession::new(Class::Middle, "Public Worker".into());
    let influencer = Profession::new(Class::Upper, "Influencer".into());;

    let construction_workers = Group::new(sc.id.clone(), construction_worker, catholic, 76_820);
    let public_workers = Group::new(rs.id.clone(), public_worker, protestant, 37_570);
    let influencers = Group::new(sp.id.clone(), influencer, agnostic, 13_350);

    sc.add_group(construction_workers);
    rs.add_group(public_workers);
    sp.add_group(influencers);

    brazil.add_state(sc);
    brazil.add_state(rs);
    brazil.add_state(sp);

    println!("{:?}", brazil);
}
