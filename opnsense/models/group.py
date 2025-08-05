from collections.abc import Iterable

from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.description import Description
from opnsense.models.gid import Gid
from opnsense.models.member import Member
from opnsense.models.name import Name
from opnsense.models.priv import Priv
from opnsense.models.scope import Scope

__NAMESPACE__ = "https://opnsense.org/config"


class Group(BaseModel):
    class Meta:
        name = "group"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    content: Iterable[object] = field(
        default_factory=list,
        metadata={
            "type": "Wildcard",
            "namespace": "##any",
            "mixed": True,
            "choices": (
                {
                    "name": "description",
                    "type": Description,
                    "namespace": "",
                },
                {
                    "name": "name",
                    "type": Name,
                    "namespace": "",
                },
                {
                    "name": "scope",
                    "type": Scope,
                    "namespace": "",
                },
                {
                    "name": "gid",
                    "type": Gid,
                    "namespace": "",
                },
                {
                    "name": "member",
                    "type": Member,
                    "namespace": "",
                },
                {
                    "name": "priv",
                    "type": Priv,
                    "namespace": "",
                },
            ),
        },
    )
