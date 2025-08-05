from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Outgoingnumtcp(BaseModel):
    class Meta:
        name = "outgoingnumtcp"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
