from collections.abc import Iterable
from typing import Optional, Union

from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.category_2 import Category2
from opnsense.models.created import Created
from opnsense.models.descr import Descr
from opnsense.models.destination import Destination
from opnsense.models.direction import Direction
from opnsense.models.disabled import Disabled
from opnsense.models.interface import Interface
from opnsense.models.ipprotocol import Ipprotocol
from opnsense.models.poolopts import Poolopts
from opnsense.models.poolopts_sourcehashkey import PooloptsSourcehashkey
from opnsense.models.protocol import Protocol
from opnsense.models.quick import Quick
from opnsense.models.source import Source
from opnsense.models.sourceport import Sourceport
from opnsense.models.statetype import Statetype
from opnsense.models.tag import Tag
from opnsense.models.tagged import Tagged
from opnsense.models.target import Target
from opnsense.models.type_mod import Type
from opnsense.models.updated import Updated

__NAMESPACE__ = "https://opnsense.org/config"


class Rule(BaseModel):
    class Meta:
        name = "rule"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    type_value: Type | None = field(
        default=None,
        metadata={
            "name": "type",
            "type": "Element",
            "namespace": "",
        },
    )
    choice: Iterable[
        (
            Descr |
            Interface |
            Ipprotocol |
            Protocol |
            Category2 |
            Destination |
            Direction |
            Poolopts |
            PooloptsSourcehashkey |
            Quick |
            Source |
            Statetype |
            Tag |
            Tagged
        )
    ] = field(
        default_factory=list,
        metadata={
            "type": "Elements",
            "choices": (
                {
                    "name": "descr",
                    "type": Descr,
                    "namespace": "",
                },
                {
                    "name": "interface",
                    "type": Interface,
                    "namespace": "",
                },
                {
                    "name": "ipprotocol",
                    "type": Ipprotocol,
                    "namespace": "",
                },
                {
                    "name": "protocol",
                    "type": Protocol,
                    "namespace": "",
                },
                {
                    "name": "category",
                    "type": Category2,
                    "namespace": "",
                },
                {
                    "name": "destination",
                    "type": Destination,
                    "namespace": "",
                },
                {
                    "name": "direction",
                    "type": Direction,
                    "namespace": "",
                },
                {
                    "name": "poolopts",
                    "type": Poolopts,
                    "namespace": "",
                },
                {
                    "name": "poolopts_sourcehashkey",
                    "type": PooloptsSourcehashkey,
                    "namespace": "",
                },
                {
                    "name": "quick",
                    "type": Quick,
                    "namespace": "",
                },
                {
                    "name": "source",
                    "type": Source,
                    "namespace": "",
                },
                {
                    "name": "statetype",
                    "type": Statetype,
                    "namespace": "",
                },
                {
                    "name": "tag",
                    "type": Tag,
                    "namespace": "",
                },
                {
                    "name": "tagged",
                    "type": Tagged,
                    "namespace": "",
                },
            ),
        },
    )
    choice_1: Iterable[Created | Sourceport | Target | Updated] = field(
        default_factory=list,
        metadata={
            "type": "Elements",
            "choices": (
                {
                    "name": "created",
                    "type": Created,
                    "namespace": "",
                },
                {
                    "name": "sourceport",
                    "type": Sourceport,
                    "namespace": "",
                },
                {
                    "name": "target",
                    "type": Target,
                    "namespace": "",
                },
                {
                    "name": "updated",
                    "type": Updated,
                    "namespace": "",
                },
            ),
        },
    )
    disabled: Disabled | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    uuid: str | None = field(
        default=None,
        metadata={
            "type": "Attribute",
        },
    )
