from collections.abc import Iterable

from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.monitor_type import MonitorType

__NAMESPACE__ = "https://opnsense.org/config"


class LoadBalancer(BaseModel):
    class Meta:
        name = "load_balancer"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    monitor_type: Iterable[MonitorType] = field(
        default_factory=list,
        metadata={
            "type": "Element",
            "namespace": "",
            "min_occurs": 1,
        },
    )
