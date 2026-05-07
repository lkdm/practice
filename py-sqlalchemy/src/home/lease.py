from sqlalchemy.orm import Mapped
from sqlalchemy.orm import mapped_column, relationship
from typing import Optional, List
from sqlalchemy import Integer, String, ForeignKey, Date, DateTime, func
from sqlalchemy.orm import Session
from shared import Base, TimestampMixin
from sqlalchemy import Integer, String, ForeignKey, Date
from datetime import datetime, date
from home.unit import Unit
from home.property import Property

class Lease(TimestampMixin, Base):
    "A lease"
    __tablename__ = "leases"

    id: Mapped[int] = mapped_column(primary_key=True)

    unit_id: Mapped[int] = mapped_column(ForeignKey(Unit.id))

    start_date: Mapped[date] = mapped_column(Date, nullable=False)
    end_date: Mapped[date] = mapped_column(Date, nullable=True)

    unit: Mapped["Unit"] = relationship(back_populates="leases")
    # TODO: How do I add a Property ORM property, without adding a direct FK relationship

    def __str__(self):
        return f"{self.name} at {self.property.address}"

def get_leases(db: Session) -> List[Lease]:
    return db.query(Lease).all()
