from sqlalchemy.orm import Mapped
from sqlalchemy.orm import mapped_column, relationship
from typing import Optional, List
from sqlalchemy import Integer, String, ForeignKey
from sqlalchemy.orm import Session
from data import Base

# Fixes: circular import issue
from typing import TYPE_CHECKING
if TYPE_CHECKING:
    from .trade import Trade

class Product(Base):
    __tablename__ = "product"
    id: Mapped[int] = mapped_column(primary_key=True, index=True, autoincrement=True)
    product_code: Mapped[str] = mapped_column(String(100))

    trades: Mapped[List["Trade"]] = relationship(back_populates="product")

def get_products(db: Session) -> List[Product]:
    return db.query(Product).all()

