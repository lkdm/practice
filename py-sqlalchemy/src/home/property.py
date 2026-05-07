from sqlalchemy.orm import Mapped
from sqlalchemy.orm import mapped_column
from typing import Optional, List
from sqlalchemy import Integer, String, ForeignKey, Date
from sqlalchemy.orm import Session
from data import Base
from sqlalchemy import Integer, String, ForeignKey, Date
from datetime import datetime, date

class Property(Base):
    "A property"
    __tablename__ = "properties"

    id: Mapped[int] = mapped_column(primary_key=True)
    address: Mapped[str] = mapped_column(String(300))

    sold_date: Mapped[date] = mapped_column(Date, nullable=True)

def get_properties(db: Session) -> List[Property]:
    return db.query(Property).all()
