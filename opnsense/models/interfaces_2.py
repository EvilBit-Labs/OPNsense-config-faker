from collections.abc import Iterable

from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

__NAMESPACE__ = "https://opnsense.org/config"


class Interfaces2(BaseModel):
    class Meta:
        name = "interfaces"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    any_element: Iterable[object] = field(
        default_factory=list,
        metadata={
            "type": "Wildcard",
            "namespace": "##any",
        },
    )
