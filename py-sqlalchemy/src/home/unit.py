from sqlalchemy.orm import Mapped
from sqlalchemy.orm import mapped_column, relationship
from typing import Optional, List
from sqlalchemy import Integer, String, ForeignKey, Date
from sqlalchemy.orm import Session
from shared import Base
from sqlalchemy import Integer, String, ForeignKey, Date
from datetime import datetime, date
from home.property import Property

class Unit(Base):
    "A unit"
    __tablename__ = "units"

    id: Mapped[int] = mapped_column(primary_key=True)
    property_id: Mapped[int] = mapped_column(ForeignKey(Property.id))
    name: Mapped[str] = mapped_column(String(32)) # TODO: Unique to property_id

    property: Mapped["Property"] = relationship(back_populates="units")

    def __str__(self):
        return f"{self.name} at {self.property.address}"

def get_units(db: Session) -> List[Unit]:
    return db.query(Unit).all()
