from collections.abc import Iterable

from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.rule import Rule

__NAMESPACE__ = "https://opnsense.org/config"


class Filter2(BaseModel):
    class Meta:
        name = "filter"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    rule: Iterable[Rule] = field(
        default_factory=list,
        metadata={
            "type": "Element",
            "namespace": "",
            "min_occurs": 1,
        },
    )
