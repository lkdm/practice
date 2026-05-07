from sqlalchemy.orm import Session
from home.property import Property, get_properties
from home.unit import Unit, get_units

def seed(db: Session):
    properties = [
        Property(address="100 Burberry Ave, Yorktown NSW, 2000"),
        Property(address="2 Victoria Crt, Jamiston NSW, 2111"),

    ]
    db.add_all(properties)
    units = [
        Unit(property=properties[0], name="Front room")
        # Trade(product=products[0], business_date=today, side=Side.B, qty=10),

    ]
    db.add_all(units)
    db.commit()

def run(db: Session):
    seed(db)
    [print(property) for property in get_properties(db)]
    [print(unit) for unit in get_units(db)]
    
