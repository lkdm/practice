from sqlalchemy.orm import Mapped
from sqlalchemy.orm import mapped_column, relationship
from typing import Optional, List
from sqlalchemy import Integer, String, ForeignKey, Date
from sqlalchemy.orm import Session
from shared import ArchivedAtMixin, Base, TimestampMixin
from sqlalchemy import Integer, String, ForeignKey, Date
from datetime import datetime, date

# Fixes: circular import issue
from typing import TYPE_CHECKING
if TYPE_CHECKING:
    from .unit import Unit

class Property(TimestampMixin, ArchivedAtMixin, Base):
    "A property"
    __tablename__ = "properties"

    id: Mapped[int] = mapped_column(primary_key=True)
    address: Mapped[str] = mapped_column(String(300))

    sold_date: Mapped[date] = mapped_column(Date, nullable=True)

    units: Mapped[List["Unit"]] = relationship(back_populates="property")

    def __str__(self):
        return f"{self.address}"

def get_properties(db: Session) -> List[Property]:
    return db.query(Property).all()
