from collections.abc import Iterable

from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.item import Item

__NAMESPACE__ = "https://opnsense.org/config"


class Sysctl(BaseModel):
    class Meta:
        name = "sysctl"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    item: Iterable[Item] = field(
        default_factory=list,
        metadata={
            "type": "Element",
            "namespace": "",
            "min_occurs": 1,
        },
    )
