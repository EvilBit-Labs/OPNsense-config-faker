from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Logqueries(BaseModel):
    class Meta:
        name = "logqueries"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
