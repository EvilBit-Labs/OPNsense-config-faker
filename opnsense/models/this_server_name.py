from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class ThisServerName(BaseModel):
    class Meta:
        name = "this_server_name"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
