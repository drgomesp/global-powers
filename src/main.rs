use crate::population::{Class, Ethnicity, Group, Profession, Religion, SubGroup};
use crate::region::{Country, State};

mod population;
mod region;

fn main() {
    let mut brazil = Country::new();

    let mut sc = State::new("SC".into(), "Santa Catarina".into());
    let mut rs = State::new("RS".into(), "Rio Grande do Sul".into());
    let mut sp = State::new("SP".into(), "Sao Paulo".into());

    let white = Ethnicity::new("White".into());
    let mixed = Ethnicity::new("Mixed".into());
    let black = Ethnicity::new("Black".into());

    let catholic = Religion::new("Catholic".into());
    let protestant = Religion::new("Protestant".into());
    let agnostic = Religion::new("Agnostic".into());

    let construction_worker = Profession::new(Class::Lower, "Construction Worker".into());
    let public_worker = Profession::new(Class::Middle, "Public Worker".into());
    let influencer = Profession::new(Class::Upper, "Influencer".into());

    let mut sc_construction_workers = Group::new(sc.id.clone(), &construction_worker);
    let mut rs_construction_workers = Group::new(rs.id.clone(), &construction_worker);
    let mut rs_public_workers = Group::new(rs.id.clone(), &public_worker);
    let mut sp_influencers = Group::new(sp.id.clone(), &influencer);

    sc_construction_workers.add_sub_group(SubGroup::new(&white, &catholic, 10_000));
    sc_construction_workers.add_sub_group(SubGroup::new(&white, &protestant, 10_000));
    sc_construction_workers.add_sub_group(SubGroup::new(&mixed, &catholic, 10_000));
    sc_construction_workers.add_sub_group(SubGroup::new(&mixed, &protestant, 10_000));
    sc_construction_workers.add_sub_group(SubGroup::new(&black, &catholic, 10_000));

    rs_construction_workers.add_sub_group(SubGroup::new(&white, &catholic, 10_000));
    rs_construction_workers.add_sub_group(SubGroup::new(&white, &protestant, 10_000));
    rs_construction_workers.add_sub_group(SubGroup::new(&white, &agnostic, 10_000));

    rs_public_workers.add_sub_group(SubGroup::new(&white, &catholic, 10_000));
    rs_public_workers.add_sub_group(SubGroup::new(&white, &protestant, 10_000));
    rs_public_workers.add_sub_group(SubGroup::new(&mixed, &catholic, 10_000));
    rs_public_workers.add_sub_group(SubGroup::new(&black, &catholic, 10_000));

    sp_influencers.add_sub_group(SubGroup::new(&white, &catholic, 10_000));
    sp_influencers.add_sub_group(SubGroup::new(&white, &protestant, 10_000));
    sp_influencers.add_sub_group(SubGroup::new(&mixed, &catholic, 10_000));
    sp_influencers.add_sub_group(SubGroup::new(&black, &catholic, 10_000));

    sc.add_group(sc_construction_workers);
    rs.add_group(rs_public_workers);
    rs.add_group(rs_construction_workers);
    sp.add_group(sp_influencers);

    brazil.add_state(&sc);
    brazil.add_state(&rs);
    brazil.add_state(&sp);

    println!("{:#?}", brazil);
}
