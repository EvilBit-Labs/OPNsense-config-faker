from decimal import Decimal

from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

__NAMESPACE__ = "https://opnsense.org/config"


class Time(BaseModel):
    class Meta:
        name = "time"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    value: Decimal = field(
        metadata={
            "required": True,
        }
    )
