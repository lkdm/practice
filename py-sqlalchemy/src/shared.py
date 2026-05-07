from sqlalchemy.orm import DeclarativeBase
from datetime import datetime
from sqlalchemy.orm import Mapped, mapped_column
from sqlalchemy import DateTime, func
from typing import Optional

# declarative base class
class Base(DeclarativeBase):
    pass


class TimestampMixin:
    """
    Adds created_at and modified_at
    """
    created_at: Mapped[datetime] = mapped_column(
        DateTime(timezone=True),
        server_default=func.now(),
        nullable=False,
    )

    modified_at: Mapped[datetime] = mapped_column(
        DateTime(timezone=True),
        server_default=func.now(),
        onupdate=func.now(),
        nullable=False,
    )


class ArchivedAtMixin:
    """
    Adds archived_at
    """
    archived_at: Mapped[Optional[datetime]] = mapped_column(
        DateTime(timezone=True),
        nullable=True,
        default=None,
    )

    @property
    def is_archived(self) -> bool:
        return self.archived_at is not None

    def archive(self) -> None:
        self.archived_at = datetime.utcnow()

    def restore(self) -> None:
        self.archived_at = None
