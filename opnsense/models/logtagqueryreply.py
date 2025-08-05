from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Logtagqueryreply(BaseModel):
    class Meta:
        name = "logtagqueryreply"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
