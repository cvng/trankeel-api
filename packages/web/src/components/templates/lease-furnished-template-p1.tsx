import { Alert, Pane } from "evergreen-ui";
import React from "react";
import { DocumentPageProps } from "../document/document";

export const LeaseFurnishedTemplatePart1: React.FunctionComponent<
  DocumentPageProps
> = ({
  form,
  getItem,
}) => {
  return (
    <Pane>
      <h2 id="contrat-type-de-location-ou-de-colocation-de-logement-meuble">
        CONTRAT TYPE DE LOCATION OU DE COLOCATION DE LOGEMENT MEUBLE
      </h2>
      <pre>
        (Soumis au titre Ier bis de la loi du 6 juillet 1989 tendant à améliorer
        les rapports locatifs<br />{" "}
        et portant modification de la loi n° 86-1290 du 23 décembre 1986)
      </pre>
      <p>
        <strong>Champ du contrat type</strong>
        : Le présent contrat type de location est applicable aux locations et
        aux colocations de logement meublé et qui constitue la résidence
        principale du preneur, à l'exception :
      </p>
      <ul>
        <li>
          des colocations formalisées par la conclusion de plusieurs contrats
          entre les locataires et le bailleur ;
        </li>
        <li>
          des locations de logement appartenant à un organisme d'habitation à
          loyer modéré et faisant l'objet d'une convention passée en application
          de l'article L.351-2 du code de la construction et de l'habitation.
        </li>
      </ul>
      <Alert
        title="Modalités d'application du contrat type"
        intent="none"
        marginBottom={32}
      >
        Le régime de droit commun en matière de baux d'habitation est défini
        principalement par la loi n° 89-462 du 6 juillet 1989 tendant à
        améliorer les rapports locatifs et portant modification de la loi n°
        86-1290 du 23 décembre 1986. L'ensemble de ces dispositions étant
        d'ordre public, elles s'imposent aux parties qui, en principe, ne
        peuvent pas y renoncer.
      </Alert>
      <p>En conséquence :</p>
      <ul>
        <li>
          le présent contrat type de location contient uniquement les clauses
          essentielles du contrat dont la législation et la réglementation en
          vigueur au jour de sa publication imposent la mention par les parties
          dans le contrat. Il appartient cependant aux parties de s'assurer des
          dispositions applicables au jour de la conclusion du contrat.
        </li>
        <li>
          au-delà de ces clauses, les parties sont également soumises à
          l'ensemble des dispositions légales et réglementaires d'ordre public
          applicables aux baux d'habitation sans qu'il soit nécessaire de les
          faire figurer dans le contrat et qui sont rappelées utilement dans la
          notice d'information qui doit être jointe à chaque contrat.
        </li>
        <li>
          les parties sont libres de prévoir dans le contrat d'autres clauses
          particulières, propres à chaque location, dans la mesure où celles-ci
          sont conformes aux dispositions législatives et réglementaires en
          vigueur. Les parties peuvent également convenir de l'utilisation de
          tout autre support pour établir leur contrat, dans le respect du
          présent contrat type.
        </li>
      </ul>
      <div style={{ pageBreakAfter: "always" }} />
      <p>
        <strong>
          Le contrat type de location ou de colocation contient les éléments
          suivants :
        </strong>
      </p>
      <pre>
        <code>
          <span className="hljs-keyword">I</span>. DESIGNATION DES PARTIES{"\n"}
        </code>
      </pre>
      <p>Le présent contrat est conclu entre les soussignés :</p>
      <ul>
        <li>
          {getItem("lender_name")} (25) désigné(s) ci-après le bailleur.
        </li>
        {
          /* {'{'}% if lender_mandatary_name %{'}'}
                <li>Le cas échéant, représenté par le mandataire :</li>
                <li>{'{'}{'{'}lender_mandatary_name{'}'}{'}'} ;</li>
                <li>Le cas échéant {'{'}{'{'}lender_mandatary_professional_data{'}'}{'}'} (26).</li>
                {'{'}% endif %{'}'} */
        }
        <li>
          {getItem("tenant_fullname")} désigné(s) ci-après le locataire.
        </li>
      </ul>
      <p>Il a été convenu ce qui suit :</p>
      <h2 id="ii-objet-du-contrat">II. OBJET DU CONTRAT</h2>
      <p>
        Le présent contrat a pour objet la location d'un logement ainsi
        déterminé :
      </p>
      <p>
        <strong>A. Consistance du logement</strong>
      </p>
      <ul>
        <li>
          localisation du logement : {getItem("property_address")}
        </li>
        <li>
          type d'habitat :
          {getItem("property_housing_type")}
        </li>
        <li>
          régime juridique de l'immeuble :
          {getItem("property_building_legal_status")}
        </li>
        <li>
          période de construction :
          {getItem("property_construction_period")}
        </li>
        <li>
          surface habitable : {getItem("property_surface")}
        </li>
        <li>
          nombre de pièces principales : {getItem("property_room_count")}
        </li>
        <li>
          le cas échéant, autres parties du logement :
          {getItem("property_other_spaces_inline")}
        </li>
        <li>
          le cas échéant, Eléments d'équipements du logement :
          {getItem("property_equipments_inline")}
        </li>
        <li>
          production chauffage :
          {getItem("property_heating_method")}
        </li>
        <li>
          modalité de production d'eau chaude sanitaire (28):
          {getItem("property_water_heating_method")}
        </li>
      </ul>
      <p>
        <strong>B. Destination des locaux</strong> :
      </p>
      {getItem("property_usage")}
      <p>
        <strong>
          C. Le cas échéant, Désignation des locaux et équipements accessoires
          de l'immeuble à usage privatif du locataire
        </strong>
        :
      </p>
      {getItem("property_tenant_private_spaces")}

      <p>
        <strong>
          D. Le cas échéant, Enumération des locaux, parties, équipements et
          accessoires de l'immeuble à usage commun
        </strong>
        :
      </p>
      {getItem("property_common_spaces")}
      <p>
        <strong>E</strong>.
        <strong>
          Le cas échéant, Equipement d'accès aux technologies de l'information
          et de la communication
        </strong>
        :
      </p>
      {getItem("property_ntic_equipments_inline")}
      <div style={{ pageBreakAfter: "always" }} />
    </Pane>
  );
};
